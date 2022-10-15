# Backrust

## What is this ? 
a rust backup program that compress the files in tar.gz format and save to a s3 bucket and send a notification via email at end of operation :)

## Why?
to learn more about Rust ❤️

# To-Do List
- [x] Create the backup filename with a date.
- [x] Pickup multiples directories to backup.
- [x] A log with more detailed info to be sent via email.
- [x] Check if there a backupfile with more than 7 days and delete it.
- [x] Send a email if something goes wrong.
- [x] Do a better error handling.
- [x] integration with s3 with aws sdk
- [ ] Refactor.