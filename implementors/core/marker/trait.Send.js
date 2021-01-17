(function() {var implementors = {};
implementors["brute_force"] = [{"text":"impl Send for Opt","synthetic":true,"types":[]}];
implementors["data_generator"] = [{"text":"impl Send for Opt","synthetic":true,"types":[]},{"text":"impl Send for Mode","synthetic":true,"types":[]}];
implementors["map_reduce"] = [{"text":"impl Send for Opt","synthetic":true,"types":[]}];
implementors["map_store"] = [{"text":"impl Send for Opt","synthetic":true,"types":[]}];
implementors["topkstr"] = [{"text":"impl Send for MapStore","synthetic":true,"types":[]},{"text":"impl Send for BruteForce","synthetic":true,"types":[]},{"text":"impl Send for MapReduce","synthetic":true,"types":[]},{"text":"impl Send for Generator","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()