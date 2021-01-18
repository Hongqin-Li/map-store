var searchIndex = JSON.parse('{\
"bruteforce":{"doc":"","i":[[3,"Opt","bruteforce","",null,null],[12,"file","","File to process.",0,null],[12,"topk","","Output k most common string.",0,null],[5,"main","","",null,[[]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"to_subset","","",0,[[],["option",4]]],[11,"is_in_subset","","",0,[[]]],[11,"to_subset_unchecked","","",0,[[]]],[11,"from_subset","","",0,[[]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"clap","","",0,[[],["app",3]]],[11,"from_clap","","",0,[[["argmatches",3]]]],[11,"augment_clap","","",0,[[["app",3]],["app",3]]],[11,"is_subcommand","","",0,[[]]]],"p":[[3,"Opt"]]},\
"data":{"doc":"Dataset genertor.","i":[[3,"Opt","data","",null,null],[12,"output","","Output file.",0,null],[12,"size","","Size of samples to generate in MB.",0,null],[12,"mode","","Mode of sample distribution.",0,null],[4,"Mode","","",null,null],[13,"Distinct","","",1,null],[13,"Identical","","",1,null],[13,"Normal","","",1,null],[5,"main","","",null,[[]]],[11,"variants","","",1,[[]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"to_subset","","",0,[[],["option",4]]],[11,"is_in_subset","","",0,[[]]],[11,"to_subset_unchecked","","",0,[[]]],[11,"from_subset","","",0,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_string","","",1,[[],["string",3]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"to_subset","","",1,[[],["option",4]]],[11,"is_in_subset","","",1,[[]]],[11,"to_subset_unchecked","","",1,[[]]],[11,"from_subset","","",1,[[]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"from_str","","",1,[[],["result",4]]],[11,"clap","","",0,[[],["app",3]]],[11,"from_clap","","",0,[[["argmatches",3]]]],[11,"augment_clap","","",0,[[["app",3]],["app",3]]],[11,"is_subcommand","","",0,[[]]]],"p":[[3,"Opt"],[4,"Mode"]]},\
"map_store":{"doc":"MapReduce-based key-value storage.","i":[[3,"MapStore","map_store","MapReduce-based key-value storage.",null,null],[3,"BatchWriter","","Writing to disk in batch by caching in memory for…",null,null],[11,"new","","Create a new [BatchWriter] with maximum cache size in…",0,[[["vec",3],["pathbuf",3]]]],[11,"write","","Write content in `buf` to file of path by `path_id`.",0,[[["vec",3]]]],[11,"flush1","","Flush cache in one file to disk.",0,[[]]],[11,"flush","","Flush all in-memory cache to disk.",0,[[]]],[11,"new","","Create a new MapStore with `nmaps` maps in directory `dir`.",1,[[]]],[11,"apply","","Apply operation `op` on `key`.",1,[[]]],[11,"map1_without_compact","","Get the key-value map in one map region without any doing…",1,[[]]],[11,"map1","","Get the key-value mapping in one map region without any…",1,[[],[["vec",3],["hashmap",3]]]],[11,"iter1","","Iterate over key-value pair in one map region.",1,[[]]],[11,"iter","","Iterator over all key-value pairs.",1,[[]]],[11,"iter_without_compaction","","Iterator over all key-value pairs without compaction on…",1,[[]]],[11,"get","","Retrieve the value associated with specified key.",1,[[],["option",4]]],[8,"Operator","","Trait representing certain operation to be applied on…",null,null],[10,"apply","","Apply this operation on `value`.",2,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"to_subset","","",1,[[],["option",4]]],[11,"is_in_subset","","",1,[[]]],[11,"to_subset_unchecked","","",1,[[]]],[11,"from_subset","","",1,[[]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"to_subset","","",0,[[],["option",4]]],[11,"is_in_subset","","",0,[[]]],[11,"to_subset_unchecked","","",0,[[]]],[11,"from_subset","","",0,[[]]],[11,"drop","","",0,[[]]],[11,"drop","","",1,[[]]],[11,"default","","",0,[[],["batchwriter",3]]]],"p":[[3,"BatchWriter"],[3,"MapStore"],[8,"Operator"]]},\
"mapreduce":{"doc":"","i":[[3,"Opt","mapreduce","",null,null],[12,"file","","File to process.",0,null],[12,"nmaps","","Number of intermediate fils of MapReduce algorithm.",0,null],[12,"topk","","Output k most common string.",0,null],[5,"main","","",null,[[]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"to_subset","","",0,[[],["option",4]]],[11,"is_in_subset","","",0,[[]]],[11,"to_subset_unchecked","","",0,[[]]],[11,"from_subset","","",0,[[]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"clap","","",0,[[],["app",3]]],[11,"from_clap","","",0,[[["argmatches",3]]]],[11,"augment_clap","","",0,[[["app",3]],["app",3]]],[11,"is_subcommand","","",0,[[]]]],"p":[[3,"Opt"]]},\
"mapstore":{"doc":"","i":[[3,"Opt","mapstore","",null,null],[12,"file","","File to process.",0,null],[12,"nmaps","","Number of intermediate fils of MapReduce algorithm.",0,null],[12,"topk","","Output k most common string.",0,null],[5,"main","","",null,[[]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"to_subset","","",0,[[],["option",4]]],[11,"is_in_subset","","",0,[[]]],[11,"to_subset_unchecked","","",0,[[]]],[11,"from_subset","","",0,[[]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"clap","","",0,[[],["app",3]]],[11,"from_clap","","",0,[[["argmatches",3]]]],[11,"augment_clap","","",0,[[["app",3]],["app",3]]],[11,"is_subcommand","","",0,[[]]]],"p":[[3,"Opt"]]},\
"topkstr":{"doc":"Solutions for top K frequent string.","i":[[3,"MapStore","topkstr","MapStore method based on MapReduce with `M = nmaps`.",null,null],[12,"nmaps","","Number of splits on keys.",0,null],[3,"BruteForce","","Brute-force method by hash table and heap.",null,null],[3,"MapReduce","","MapReduce method with `M = nmaps`.",null,null],[12,"nmaps","","Number of splits on keys.",1,null],[3,"MinkSet","","Data structure to maintain k smallest elements in a set.",null,null],[4,"Generator","","Dataset generator.",null,null],[13,"Distinct","","All strings are different.",2,null],[13,"Normal","","Distribution of strings follows pareto distribution",2,null],[13,"Identical","","All strings are same.",2,null],[11,"generate","","Generate dataset of `size` MB.",2,[[]]],[11,"new","","Create a [MinkSet] to maintain k smallest elements in it.",3,[[]]],[11,"insert","","Insert an element with value x in the set.",3,[[]]],[11,"into_sorted_vec","","Consumes the [MinkSet] and returns a vector in sorted…",3,[[],["vec",3]]],[8,"Solution","","A trait of solutions.",null,null],[10,"solve","","Solve a top k frequent string problem with k of `topk` and…",4,[[],[["string",3],["hashmap",3]]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"to_subset","","",0,[[],["option",4]]],[11,"is_in_subset","","",0,[[]]],[11,"to_subset_unchecked","","",0,[[]]],[11,"from_subset","","",0,[[]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"vzip","","",5,[[]]],[11,"to_subset","","",5,[[],["option",4]]],[11,"is_in_subset","","",5,[[]]],[11,"to_subset_unchecked","","",5,[[]]],[11,"from_subset","","",5,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"to_subset","","",1,[[],["option",4]]],[11,"is_in_subset","","",1,[[]]],[11,"to_subset_unchecked","","",1,[[]]],[11,"from_subset","","",1,[[]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"to_subset","","",3,[[],["option",4]]],[11,"is_in_subset","","",3,[[]]],[11,"to_subset_unchecked","","",3,[[]]],[11,"from_subset","","",3,[[]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"to_subset","","",2,[[],["option",4]]],[11,"is_in_subset","","",2,[[]]],[11,"to_subset_unchecked","","",2,[[]]],[11,"from_subset","","",2,[[]]],[11,"solve","","",5,[[],[["string",3],["hashmap",3]]]],[11,"solve","","",1,[[],[["string",3],["hashmap",3]]]],[11,"solve","","",0,[[],[["string",3],["hashmap",3]]]]],"p":[[3,"MapStore"],[3,"MapReduce"],[4,"Generator"],[3,"MinkSet"],[8,"Solution"],[3,"BruteForce"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);