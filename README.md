## r2t-i7h Automatic i18n
This project is inspired by [RimoChan/i7h](https://github.com/RimoChan/i7h).

Have you encountered words like i18n or k8s? i18n is an abbreviation of "internationalization".

Who have invented this kind of abbrebiation? That's sick.

This program will turn all english word into this abbreviated form.

### Example
Before:
```
Alice was beginning to get very tired of sitting by her sister on the bank, 
and of having nothing to do: once or twice she had peeped into the book her 
sister was reading, but it had no pictures or conversations in it, `and what 
is the use of a book,' thought Alice `without pictures or conversation?'

So she was considering in her own mind (as well as she could, for the hot day 
made her feel very sleepy and stupid), whether the pleasure of making a 
daisy-chain would be worth the trouble of getting up and picking the daisies, 
when suddenly a White Rabbit with pink eyes ran close by her. 
```
After (with threshold of 5):
```
A3e was b7g to get very t3d of s5g by her s4r on the bank,
and of h4g n5g to do: once or t3e she had p4d into the book her
s4r was r5g, but it had no p6s or c11s in it, `and what
is the use of a book,' t5t A3e `w5t p6s or c10n?'

So she was c9g in her own mind (as well as she c3d, for the hot day
made her feel very s4y and s4d), w5r the p6e of m4g a
d3y-c3n w3d be w3h the t5e of g5g up and p5g the d5s,
when s6y a W3e R4t with pink eyes ran c3e by her.
```

### Usage
```bash 
./r2t-i7h [arguments..] [file_name]
```

You can specifiy the file to read. If no file is specified, this program will read from stdin. 
```bash
./r2t-i7h text.txt
```

You can also specify the length threshold by `-t` or `--threshold`. Word that's shorter than threshold will not be abbreviated.
```bash
./r2t-i7h -t 5 text.txt
```