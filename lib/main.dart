import 'package:flutter/material.dart';
import 'home_page.dart';
import 'theme.dart';

void main() {
  runApp(NotesApp());
}

class NotesApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Notes App',
      darkTheme: darkTheme,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true, // Enable Material 3 design
        textTheme: TextTheme(
          bodyMedium: TextStyle(fontSize: 18, color: Colors.deepPurple),  // Updated from bodyText2
        ),
      ),
      //themeMode: ThemeMode.system, // Can be ThemeMode.light or ThemeMode.dark
      home: PublicKeyInputPage(),
    );
  }
}

// Public Key Input Screen
class PublicKeyInputPage extends StatefulWidget {
  @override
  _PublicKeyInputPageState createState() => _PublicKeyInputPageState();
}

class _PublicKeyInputPageState extends State<PublicKeyInputPage> {
  final TextEditingController _publicKeyController = TextEditingController();
  String _errorMessage = '';

  void _submitPublicKey() {
    String publicKey = _publicKeyController.text.trim();

    // Basic validation
    if (publicKey.isEmpty) {
      setState(() {
        _errorMessage = 'Please enter a public key';
      });
    } else {
      // Navigate to the notes page
      Navigator.push(
        context,
        MaterialPageRoute(
          builder: (context) => NotesHomePage(publicKey: publicKey),
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Enter Public Key'),
        centerTitle: true,
        elevation: 4.0,
      ),
      body: Padding(
        padding: const EdgeInsets.all(24.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: <Widget>[
            // Title text
            Text(
              'Welcome to Cross Clip',
              style: TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.bold,
                color: Colors.deepPurpleAccent,
              ),
              textAlign: TextAlign.center,
            ),
            SizedBox(height: 10),
            Text(
              'Please enter your public key to continue:',
              style: TextStyle(fontSize: 16),
              textAlign: TextAlign.center,
            ),
            SizedBox(height: 20),

            // Public Key Text Field with a card-like UI
            Card(
              elevation: 4.0,
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(12),
              ),
              child: Padding(
                padding: const EdgeInsets.all(8.0),
                child: TextField(
                  controller: _publicKeyController,
                  decoration: InputDecoration(
                    hintText: 'Enter your public key',
                    border: OutlineInputBorder(),
                  ),
                ),
              ),
            ),
            SizedBox(height: 20),

            // Error message
            if (_errorMessage.isNotEmpty)
              Text(
                _errorMessage,
                style: TextStyle(color: Colors.red),
                textAlign: TextAlign.center,
              ),

            // Submit button
            SizedBox(height: 20),
            ElevatedButton(
              onPressed: _submitPublicKey,
              style: ElevatedButton.styleFrom(
                padding: EdgeInsets.symmetric(vertical: 16),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(10),
                ),
                backgroundColor: Colors.deepPurple
              ),
              child: Text(
                'Submit',
                style: TextStyle(fontSize: 18, color: Colors.white),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
