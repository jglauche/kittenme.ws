# kittenme.ws
Hacking together my new personal site kittenme.ws

I am used to use haml from ruby web development and wanted to see if I can use it in rust as well. 
I'm using the haml-rs library which has a few shortcomings and doesn't like being mixed with the rocket template environment. 

I hacked together a way to use haml subtemplates, and glue them together (currently at request time) in a hacky way to get it running.
