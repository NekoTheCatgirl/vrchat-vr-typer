# Vrchat VR typer
A auto splitting typing utility for vrchat users who want to use hand tracking while also being able to use their physical keyboard to text their friends (usefull for mutes who know how to touch type)

## Why was this made?
The main reason i made this was because i was enjoying a good activity where you answer questions in a circle. And being a mute in vrchat, i found it hard to answer the questions propperly, thus i want a way to type easier, especially when i am not using any controllers.

I noticed aswell that vrchat would only recognize the physical keyboards input if i had the steamvr menu up, making my hand tracking data not transmit to vrchat.

"There are this and that already" My answer is, i like learning ^^ And thus i made something that let me both learn, and have fun at the same time. While also solving my problem.

## How to use
If you are running the application on the same machine that vrchat is running on, follow these instructions:
1. Ensure that you have [OSC enabled in vrchat](https://docs.vrchat.com/docs/osc-overview#enabling-it)
2. Launch the application (it auto connects)

If you are running vrchat on a different machine or on a quest, follow these instructions:
1. Ensure that you have [OSC enabled in vrchat](https://docs.vrchat.com/docs/osc-overview#enabling-it)
2. Get the IP of the device running vrchat
3. Run the application with the argument `--ip "<ip>"`

## Running on a non windows machine
As this was compiled only on windows, you will have to compile it yourself on your machine you want to run it on. Simply use cargo to build the application `cargo build --release`