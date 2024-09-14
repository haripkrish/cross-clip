import 'package:flutter/material.dart';
import 'package:cross_clip/src/rust/api/simple.dart';
import 'package:cross_clip/src/rust/frb_generated.dart';
//
// import 'package:flutter/material.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Cross Clip Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Cross Clip Demo'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);
  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  late Stream<int> ticks;

  @override
  void initState() {
    super.initState();
    ticks = tick();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text("Time since starting Rust stream"),
            StreamBuilder<int>(
              stream: ticks,
              builder: (context, snap) {
                final style = Theme.of(context).textTheme.headlineMedium;
                final error = snap.error;
                if (error != null)
                  return Tooltip(
                      message: error.toString(),
                      child: Text('Error', style: style));

                final data = snap.data;
                if (data != null) return Text('$data second(s)', style: style);

                return const CircularProgressIndicator();
              },
            )
          ],
        ),
      ),
    );
  }
}