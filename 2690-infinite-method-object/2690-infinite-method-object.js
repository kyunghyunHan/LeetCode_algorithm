/**
 * @return {Object}
 */
var createInfiniteObject = function() {
    return new Proxy({},{
        get:(_,key)=>{
            return ()=>String(key)
        }
    })
    
};

/**
 * const obj = createInfiniteObject();
 * obj['abc123'](); // "abc123"
 */