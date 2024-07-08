## motivation and kudos
I'd be lost without text and so this project won't start (or at least finish) without a 
thought dump, my old friend.

Motivation behind this whole project was from a challenge from a friend of mine, Zach Birenbaum 
for me to see how hard Rust actually is. (Some background, the only Rust I've done was using
ChatGPT to make a translucent BASIC text editor, of which I have almost no recollection. Outside
of the YouTube videos and random mentioning of something the language has, I know almost nothing.)

Further motivation for the implementation comes from my friend Thor, who ran an experiment testing
whether prototyping a program in Python before writing in Rust would save one time, and yes, it did.
Since it worked for him, a professional Rust dev, I imagine it couldn't hurt for me.
(See experiment here: https://thork.net/posts/20211015_how_effective_prototyping_euler50/)

What follows, may not have any use to you. You are warned. This will consist of me writing down
mostly when my confusions arise and walk through what's going through my head in order to get though it.
I'd love to know if this types of doc is useful, and if so, message me on github.

## criteria
- Using ChatGPT for this won't help me learn. I will be using google / SO for learning how to do this all.

## thoughts
(ooh. I could take these thoughts and turn it into a blog post.)
- (2024.07.07__22.33.03) I don't think I'll be able to finish this tonight, so I'll take out one or two chunks at a time.
- I started with just having a list to add to, so let's get a hello world, a list to add multiple things to (print it), then call it a night
- Updated.
- and I've got hello world. (22:41:10) great.
- ok, diff between macro and a function is using ! for the macros.
- ok, have an executable file after compiling. I've seen this with c++
- for this I don't think I need to read more about cargo. I'll come back to this.
- It looks like I'll need to use a vector (instead of array).
- awesome, I have a list I can add things to, and then print them out. (22:50:02) 
- What would be the next things?
    - 
- let's test the function calling (https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
    - need to figure out how to accept a vector for the function
    - yeah getting stuck on this. brb (23:10:01) -- (23:19:18) 
    - ok, figured it out. (23:28:35). I was able to use the compiler hints to figure it out
        - vec.push to add to the vector, gives the vector a type Vec<$str>, so I needed to set the type hints to accept that. And I sleep now.
- I ended up finding this guide about primitive types, macros, modules; probably helpful in the future (https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/primitive.char.html)
