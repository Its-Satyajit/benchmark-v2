import 'dart:io';
import 'dart:convert';
import 'dart:typed_data';

class SimpleSha256 {
  static String hash(String input) {
    final bytes = utf8.encode(input);
    final k = [
      0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
      0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
      0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
      0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
      0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
      0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
      0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
      0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
    ];

    int rotr(int x, int n) => ((x >> n) | (x << (32 - n))) & 0xFFFFFFFF;
    int ch(int x, int y, int z) => (x & y) ^ (~x & z);
    int maj(int x, int y, int z) => (x & y) ^ (x & z) ^ (y & z);
    int sig0(int x) => rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22);
    int sig1(int x) => rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25);
    int gamma0(int x) => rotr(x, 7) ^ rotr(x, 18) ^ (x >> 3);
    int gamma1(int x) => rotr(x, 17) ^ rotr(x, 19) ^ (x >> 10);

    final bitLen = bytes.length * 8;
    final padLen = (bytes.length % 64 < 56) ? (56 - bytes.length % 64) : (120 - bytes.length % 64);
    final totalLen = bytes.length + padLen + 8;
    final buffer = Uint8List(totalLen);
    buffer.setRange(0, bytes.length, bytes);
    buffer[bytes.length] = 0x80;
    
    final byteData = ByteData.view(buffer.buffer);
    byteData.setUint64(totalLen - 8, bitLen, Endian.big);

    int h0 = 0x6a09e667, h1 = 0xbb67ae85, h2 = 0x3c6ef372, h3 = 0xa54ff53a;
    int h4 = 0x510e527f, h5 = 0x9b05688c, h6 = 0x1f83d9ab, h7 = 0x5be0cd19;

    final w = List<int>.filled(64, 0);

    for (int chunk = 0; chunk < totalLen; chunk += 64) {
      for (int i = 0; i < 16; i++) {
        w[i] = byteData.getUint32(chunk + i * 4, Endian.big);
      }
      for (int i = 16; i < 64; i++) {
        w[i] = (gamma1(w[i - 2]) + w[i - 7] + gamma0(w[i - 15]) + w[i - 16]) & 0xFFFFFFFF;
      }

      int a = h0, b = h1, c = h2, d = h3, e = h4, f = h5, g = h6, h = h7;

      for (int i = 0; i < 64; i++) {
        final t1 = (h + sig1(e) + ch(e, f, g) + k[i] + w[i]) & 0xFFFFFFFF;
        final t2 = (sig0(a) + maj(a, b, c)) & 0xFFFFFFFF;
        h = g; g = f; f = e; e = (d + t1) & 0xFFFFFFFF;
        d = c; c = b; b = a; a = (t1 + t2) & 0xFFFFFFFF;
      }

      h0 = (h0 + a) & 0xFFFFFFFF; h1 = (h1 + b) & 0xFFFFFFFF;
      h2 = (h2 + c) & 0xFFFFFFFF; h3 = (h3 + d) & 0xFFFFFFFF;
      h4 = (h4 + e) & 0xFFFFFFFF; h5 = (h5 + f) & 0xFFFFFFFF;
      h6 = (h6 + g) & 0xFFFFFFFF; h7 = (h7 + h) & 0xFFFFFFFF;
    }

    return [h0, h1, h2, h3, h4, h5, h6, h7]
        .map((x) => x.toRadixString(16).padLeft(8, '0'))
        .join();
  }
}

void main(List<String> args) {
  String? replayPath;
  bool isStress = false;
  bool isGui = false;
  int iterations = 20;

  for (int i = 0; i < args.length; i++) {
    if (args[i] == '--replay' && i + 1 < args.length) {
      replayPath = args[i + 1];
    } else if (args[i] == '--stress') {
      isStress = true;
    } else if (args[i] == '--gui' || args[i] == '--gui-jank') {
      isGui = true;
    } else if (args[i] == '--iterations' && i + 1 < args.length) {
      iterations = int.tryParse(args[i + 1]) ?? 20;
    }
  }

  if (replayPath == null) {
    stderr.writeln('Error: Missing --replay <path>');
    exit(1);
  }

  final parseWatch = Stopwatch()..start();
  final fileContent = File(replayPath).readAsStringSync();
  final data = jsonDecode(fileContent) as Map<String, dynamic>;
  parseWatch.stop();
  final parseMs = parseWatch.elapsedMicroseconds / 1000.0;

  final replayWatch = Stopwatch()..start();
  final sb = StringBuffer();

  final steps = (data['steps'] as List<dynamic>?) ?? [];
  final config = data['configuration'] as Map<String, dynamic>?;
  int totalSteps = 0;
  int totalCards = 0;
  int totalActs = 0;
  int actualIters = (isStress || isGui) ? iterations : 1;

  final frameTimes = <double>[];

  for (int it = 0; it < actualIters; it++) {
    if (isStress || isGui) sb.write('iter:$it;');
    if (config != null && config['seed'] != null) {
      sb.write('seed:${config['seed']};');
    }

    for (int i = 0; i < steps.length; i++) {
      final frameWatch = Stopwatch()..start();
      final stepBatch = steps[i] as List<dynamic>;
      totalSteps++;

      for (int j = 0; j < stepBatch.length; j++) {
        final step = stepBatch[j] as Map<String, dynamic>;
        final action = step['action'] as List<dynamic>?;
        if (action != null && action.isNotEmpty) {
          totalActs += action.length;
          final actStr = jsonEncode(action);
          if (isStress || isGui) {
            sb.write('act:$it:$i:$j:$actStr;');
          } else {
            sb.write('act:$i:$j:$actStr;');
          }
        }

        final obs = step['observation'] as Map<String, dynamic>?;
        if (obs != null) {
          final current = obs['current'] as Map<String, dynamic>?;
          if (current != null) {
            final players = (current['players'] as List<dynamic>?) ?? [];
            for (int p = 0; p < players.length; p++) {
              final player = players[p] as Map<String, dynamic>;
              final deckLen = ((player['deck'] as List<dynamic>?) ?? []).length;
              final handLen = ((player['hand'] as List<dynamic>?) ?? []).length;
              final actLen = ((player['active'] as List<dynamic>?) ?? []).length;
              final benchLen = ((player['bench'] as List<dynamic>?) ?? []).length;
              totalCards += deckLen + handLen;
              sb.write('p:$p:d$deckLen:h$handLen:a$actLen:b$benchLen;');
            }
          }
        }

        final status = step['status'] as String?;
        if (status != null) {
          sb.write('st:$status;');
        }
      }

      if (isGui) {
        frameWatch.stop();
        frameTimes.add(frameWatch.elapsedMicroseconds / 1000.0);
      }
    }
  }

  if (isGui) {
    sb.write('final_gui:steps=$totalSteps:cards=$totalCards:acts=$totalActs');
  } else if (isStress) {
    sb.write('final_stress:steps=$totalSteps:cards=$totalCards:acts=$totalActs');
  } else {
    sb.write('final:steps=$totalSteps:cards=$totalCards:acts=$totalActs');
  }

  final checksum = SimpleSha256.hash(sb.toString());
  replayWatch.stop();
  final replayMs = replayWatch.elapsedMicroseconds / 1000.0;
  final totalMs = parseMs + replayMs;
  final stepsPerSec = replayMs > 0 ? (totalSteps / (replayMs / 1000.0)) : 0.0;

  final output = <String, dynamic>{
    'target': 'flutter-dart-desktop',
    'steps_processed': totalSteps,
    'parse_duration_ms': (parseMs * 100).round() / 100,
    'replay_duration_ms': (replayMs * 100).round() / 100,
    'total_duration_ms': (totalMs * 100).round() / 100,
    'steps_per_sec': (stepsPerSec * 100).round() / 100,
    'checksum': checksum,
  };

  if (isGui && frameTimes.isNotEmpty) {
    int jankCount = 0;
    double maxFt = 0.0;
    for (final ft in frameTimes) {
      if (ft > 16.667) jankCount++;
      if (ft > maxFt) maxFt = ft;
    }

    frameTimes.sort();
    final onePctIdx = (frameTimes.length * 0.99).floor().clamp(0, frameTimes.length - 1);
    final zeroPointOneIdx = (frameTimes.length * 0.999).floor().clamp(0, frameTimes.length - 1);

    final onePctMs = frameTimes[onePctIdx] > 0 ? frameTimes[onePctIdx] : 0.001;
    final zeroPointOneMs = frameTimes[zeroPointOneIdx] > 0 ? frameTimes[zeroPointOneIdx] : 0.001;

    final avgFps = frameTimes.length / (replayMs / 1000.0);
    final onePctFps = 1000.0 / onePctMs;
    final zeroPointOneFps = 1000.0 / zeroPointOneMs;
    final jankPct = (jankCount / frameTimes.length) * 100.0;

    output['total_frames_rendered'] = frameTimes.length;
    output['avg_fps'] = (avgFps * 10).round() / 10;
    output['one_percent_low_fps'] = (onePctFps * 10).round() / 10;
    output['zero_point_one_percent_low_fps'] = (zeroPointOneFps * 10).round() / 10;
    output['jank_frame_count'] = jankCount;
    output['jank_percentage'] = (jankPct * 100).round() / 100;
    output['max_frame_time_ms'] = (maxFt * 100).round() / 100;
  }

  print(jsonEncode(output));
}
