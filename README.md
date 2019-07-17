# rxrs [![Build Status](https://travis-ci.org/aleics/rxrs.svg?branch=master)](https://travis-ci.org/aleics/rxrs)  [![License](https://img.shields.io/crates/l/cage.svg)](https://opensource.org/licenses/Apache-2.0)  
Reactive Extensions Library for Rust.

**This is a work in progress library. Do not use it for production environments.**

## Observable
### Create
For creating new Observable instances, the library provides a number of creation operators:
 * [`of`](https://github.com/aleics/rxrs/blob/372a3111dba493777d3b5847503395025aab1ea9/src/observable.rs#L46): creates a
 finite number of observables with a defined value:
 
    ```rust
    let observable = of(&[1, 2, 3]);
    ```
   
 * [`interval`](https://github.com/aleics/rxrs/blob/372a3111dba493777d3b5847503395025aab1ea9/src/observable.rs#L56):
 creates an infinite observable that emits sequential numbers every specified interval of time:
 
     ```rust
     let observable = interval(1000); // every 1000 ms (1s)
     ```
