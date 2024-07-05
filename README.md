# clai

This is a simple command line based program built using [`Rust`](https://www.rust-lang.org/) and [`rust-genai`](https://github.com/jeremychone/rust-genai).

By default, the program uses [`gemini`](https://developers.googleblog.com/en/gemini-15-pro-and-15-flash-now-available/) AI model. But this can be changed in talk_to_ai.rs file. The length of your reponse can also be adjusted in this file. 


## how to run

* Navigate to the location of the main.rs file and run this command in your terminal

```
export GEMINI_API_KEY="your_api_key"
```

* Then run this to make your query


```
cargo run q "your query"
```

* By default the set constraint is "Answer with at most five sentences.". Since, it is a terminal based AI program, it's best to keep answers concise. However to change this by adding one more argument. Like so:

```
cargo run q "your query" -c "your custom constraint"
```


## state management 

Conversation between the user and the AI is saved locally and is managed using [`Serde JSON`](https://github.com/serde-rs/json). The saved conversation, is only used to give the AI context about the previous questions. To delete the saved conversation use the command below.

```
cargo run d
```

