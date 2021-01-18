(function() {var implementors = {};
implementors["bruteforce"] = [{"text":"impl Unpin for Opt","synthetic":true,"types":[]}];
implementors["data"] = [{"text":"impl Unpin for Opt","synthetic":true,"types":[]},{"text":"impl Unpin for Mode","synthetic":true,"types":[]}];
implementors["map_store"] = [{"text":"impl&lt;V, O&gt; Unpin for MapStore&lt;V, O&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for BatchWriter","synthetic":true,"types":[]}];
implementors["mapreduce"] = [{"text":"impl Unpin for Opt","synthetic":true,"types":[]}];
implementors["mapstore"] = [{"text":"impl Unpin for Opt","synthetic":true,"types":[]}];
implementors["topkstr"] = [{"text":"impl Unpin for MapStore","synthetic":true,"types":[]},{"text":"impl Unpin for BruteForce","synthetic":true,"types":[]},{"text":"impl Unpin for MapReduce","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Unpin for MinkSet&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for Generator","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()