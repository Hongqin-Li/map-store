<h1 align="center">
    MapStore<br/>
</h1>

<p align="center">A MapReduce-based key-value storage, optimized for efficiency.</p>

<div align="center">
    <a href="../../actions"><img src="../../workflows/CI/badge.svg" alt="CI Status" style="max-width:100%;"></a>
    <a href="../../actions"><img src="../../workflows/Deploy/badge.svg" alt="Deploy Status" style="max-width:100%;"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License" style="max-width:100%;"></a>
</div>

<br/><br/>


- [ ] MapStore doc and test
- [ ] Report markdown
- [ ] Github CI and documentation.


## Solution

- [x] (Tested) Solution 1: brute-force Hash table + heap

- [x] Solution 2: MapReduce

- [ ] Solution 3: MapStore

## Optimization

Optimize according to [Flamegraph](https://github.com/flamegraph-rs/flamegraph).

### Rust IO

avoid duplicated open and close.
avoid utf-8 string, use `vec<u8>` instead

### Batch Append and Read Buffer


### I/O and CPU Thread


## Dataset

[Url Dataset](https://www.kaggle.com/teseract/urldataset)

1. Normal(1GB): 
2. Distinct(1GB): 20,000,000 distinct strings with length 50.
3. Identical(1GB): 

