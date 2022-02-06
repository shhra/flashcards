# Flashcards

Since it's easy to create notes in org-mode and difficult to create flashcards, this app tries to ease the process of making cards!

For your notes add a `context` tag. And it will be parsed as context.

For the questions related to that content, create questions and answer as below

```
* Note title :context:
  This is a note.
  
  ** What is this?. :card:
  *** A note.
```

This is just the beginning of this application. Further features are on the way. Stay tuned! 

If you want to use it clone the repo and use cargo to build!

```bash
cargo run --release
```

The app will prompt you to import some cards! Feel free to add tags to your org cards and import them. 

Any other behaviors and the app will crash by default! Do file bug reports in such cases. 
