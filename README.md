
# Port sniffer

A command line tool that lists all the open ports in a system.


## Run Locally

Run `cargo run` to run the app, run `cargo build` to build an executable file.

For Eg:

`cargo run -- -t 1000 192.168.0.106`

Here

`-t` flag is for number of threads.

`1000` Number for threads.

`192.168.0.106` IPV4


## Lessons Learned

Main motive behind making this project was to create a CLI application after reading the [book](https://doc.rust-lang.org/book/).
Various concepts were used like multithreading, pattern matching, working with TCP etc.

Specail Thanks to [Tensor Programming](https://www.youtube.com/watch?v=-Jp7sabBCp4&list=PLJbE2Yu2zumDD5vy2BuSHvFZU0a6RDmgb&ab_channel=TensorProgramming)
for such amazing tutorial.


## Support

Feel free to use it and if you like this don't forget to give a ⭐

Thanks!

## License

[MIT](https://choosealicense.com/licenses/mit/)

