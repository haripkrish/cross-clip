import 'package:flutter/material.dart';
import 'package:cross_clip/src/rust/api/simple.dart';
import 'package:cross_clip/src/rust/frb_generated.dart';
import 'second_page.dart';
//
// import 'package:flutter/material.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

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
  const MyHomePage({super.key, required this.title});
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
                if (error != null) {
                  return Tooltip(
                      message: error.toString(),
                      child: Text('Error', style: style));
                }

                final data = snap.data;
                if (data != null) return Text('$data second(s)', style: style);

                return const CircularProgressIndicator();
              },
            ),
            ElevatedButton(
              onPressed: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (context) => const SecondPage()),
                );
              },
              child: Text('Go to Second Page\n Result1: `${test4()}`'),
            ),
          ],
        ),
      ),
    );
  }
}