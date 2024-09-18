## how to run it

```bash
$ cargo run -- --data "data.csv" --output "out_qrs/"
```


## FORMAT OF INPUT CSV FILE

```csv
fullname, email,
John Doe, Johndoemail@gmail.com,
```


## OUTPUT QR CODES

It will create a new directory based on your `--output` argument and save the QR codes there with numbers starting from 1 to n.
