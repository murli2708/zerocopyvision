// ZeroCopyVision — Flutter entrypoint (skeleton).
//
// The camera stream hands each frame's pointer to the Rust core. Wire the actual
// FFI call after running `flutter_rust_bridge_codegen generate`.

import 'package:flutter/material.dart';

void main() => runApp(const ZeroCopyApp());

class ZeroCopyApp extends StatelessWidget {
  const ZeroCopyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'ZeroCopyVision',
      theme: ThemeData.dark(useMaterial3: true),
      home: const CameraView(),
    );
  }
}

class CameraView extends StatelessWidget {
  const CameraView({super.key});

  @override
  Widget build(BuildContext context) {
    // TODO (Phase 3):
    //  1. Start the camera with startImageStream.
    //  2. For each CameraImage, pass plane pointer + width/height/stride/format
    //     to the generated `processFrame` binding (no copy).
    //  3. Paint FrameResult detections as an overlay.
    return Scaffold(
      appBar: AppBar(title: const Text('ZeroCopyVision')),
      body: const Center(
        child: Text('TODO: camera stream → Rust process_frame → overlay'),
      ),
    );
  }
}
