
# CLI-APP

CLI app written in RUST. Main pourpuse was to create real time exchege system for currencys. App implements all of the `ISO 4217` currencys.



## RUN

All necessary is build in. ALl you have to do is:
- build app
```bash
cargo build
```
- and RUN
```bash
cargo run
```
## API
I've used free tier Exchange Rate API `https://www.exchangerate-api.com/`. 

If you want use anather plan or use your keys, you need to change `URL_BASE` variable to:
```bash
https://v6.exchangerate-api.com/v6/YOUR-API-KEY/latest/
```
## Commands
List of all commands:
- list 
- list cur
- exit
- cur1 cur2 qu
- help

For specific informations and usage of commands use `help`.