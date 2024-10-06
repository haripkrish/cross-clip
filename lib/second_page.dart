import 'package:flutter/material.dart';
import 'package:cross_clip/src/rust/api/simple.dart';
import 'main.dart';

class SecondPage extends StatelessWidget {
  const SecondPage({super.key});

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
  late Stream<NoteStorage> note_event_stream;

  @override
  void initState() {
    super.initState();
    note_event_stream = notesEventStream();
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
            StreamBuilder<NoteStorage>(
              stream: note_event_stream,
              // builder: (context, data) {
              //   final style = Theme.of(context).textTheme.headlineMedium;
              //   if (data.hasData) {
              //     return Text('${data.data.toString()} second(s)', style: style);
              // }
              builder: (context, snap) {
                final style = Theme.of(context).textTheme.headlineMedium;
                final error = snap.error;
                if (error != null) {
                  return Tooltip(
                      message: error.toString(),
                      child: Text('Error', style: style));
                }

                final data = snap.data;
                if (data != null) return Text('${data.data} second(s)', style: style);
                if (data == null) return Text('$note_event_stream', style: style);


                return const CircularProgressIndicator();
              },
            ),
            ElevatedButton(
              onPressed: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (context) => const MyApp()),
                );
              },
              child: const Text('Go to first Page'),
            ),
          ],
        ),
      ),
    );
  }
}