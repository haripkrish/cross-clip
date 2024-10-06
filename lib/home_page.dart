import 'package:flutter/material.dart';
import 'main.dart';
import 'theme.dart';

class NotesHomePage extends StatelessWidget {
  final String publicKey;

  NotesHomePage({required this.publicKey});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Cross Clip Home Page',
      darkTheme: darkTheme,
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: HomePage(title: 'Cross Clip homepage', publicKey: publicKey),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key, required this.title, required this.publicKey});

  final String title;
  final String publicKey;

  @override
  State<HomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<HomePage> {
  //late Stream<NoteStorage> note_event_stream;

  @override
  void initState() {
    super.initState();
    //note_event_stream = notesEventStream();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Notes page'),
      ),
      body: Center(
        child: Column(
          children: <Widget>[
            Text('Public Key: ${widget.publicKey}'),
            ElevatedButton(
              onPressed: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (context) => NotesApp()),
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
