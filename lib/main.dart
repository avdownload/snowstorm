import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:rinf/rinf.dart';
import './messages/all.dart';

void main() async {
  await initializeRust(assignRustSignal);
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.red),
      ),
      home: const MyHomePage(title: 'snowstorm'),
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
  int _counter = 0;
  List<Song> songs = [];

  void _incrementCounter() {
    setState(() {
      _counter++;
    });
  }

  @override
  void initState() {
    // TODO: implement initState
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      getInitialSongs();
    });
  }

  void getInitialSongs() async {
   await for (final song in Songs.rustSignalStream) {
    setState(() {
      songs =     song.message.songs;

    });
   }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text(widget.title)),
      body: Center(
   

           child:ListView.builder(itemBuilder: (context, index) {               songs.toSet().toList();

   return  ListTile(title: Text(songs[index].title ?? "error null data"),
    trailing: Text(songs[index].artist ?? "error null data"),
    subtitle: Text(songs[index].album ?? "error null data"),
  );
  },itemCount: songs.length,
  )),

  
  floatingActionButton: FloatingActionButton(
        onPressed: () async {
          getPath();
          await for ( RustSignal<Songs> item in Songs.rustSignalStream) {
             setState(() {
                songs = item.message.songs;
             });
             print(songs);
             print(item.message);
          }
        },
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}

void getPath() async {
  String? path = await FilePicker.platform.getDirectoryPath();
  FolderPath(path: path).sendSignalToRust();
}