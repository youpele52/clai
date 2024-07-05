# clai

this is a simple command line based program built using [`Rust`](https://www.rust-lang.org/) and [`rust-genai`](https://github.com/jeremychone/rust-genai).

by default, the program uses [`gemini`](https://developers.googleblog.com/en/gemini-15-pro-and-15-flash-now-available/) AI model. But this can be changed in talk_to_ai.rs file. The length of your reponse can also be adjusted in this file. 


## installation

* install [`rust`](https://www.rust-lang.org/tools/install)
* navigate to the project's main folder to build the program using 

```
cargo build --release
```

to use the program already, you can jump to [how to run](#how-to-run).

### run everywhere
if you want to run the program anywhere in the terminal rather than being tied to the program's main folder, you will have to add the program's binary's directory to your PATH.

* the binary can be found in clai/target/release, however, after you get to /release folder, you'll need to get the full/release/path using

```
pwd
```

* next, is to run the following commands. Don't forget to replace .zshrc with .bashrc if you are using bash

```
echo 'export PATH="$PATH:/path/to/your/binary/directory"' >> ~/.zshrc   &&

source ~/.zshrc

```

* to use the program, just like it is in [how to run](#how-to-run), first export your key, however to make queries or any other commands, replace 'cargo run' with 'clai'. 

eg
```
clai q "your query"
```


## how to run

* navigate to the project's main folder and run this command in your terminal

```
export GEMINI_API_KEY="your_api_key"
```

* then run this to make your query


```
cargo run q "your query"
```

* by default the set constraint is "Answer with at most five sentences.". Since, it is a terminal based AI program, it's best to keep answers concise. However to change this by adding one more argument. Like so:

```
cargo run q "your query" -c "your custom constraint"
```


## state management 

conversations between the user and the AI is saved locally and is managed using [`Serde JSON`](https://github.com/serde-rs/json). The saved conversation, is only used to give the AI context about the previous questions. To delete the saved conversations use the command below.

```
cargo run d
```

/Users/youpele/DevWorld/clai/target/release/clai

echo 'export PATH="$PATH:/Users/youpele/DevWorld/clai/target/release"' >> ~/.zshrc

