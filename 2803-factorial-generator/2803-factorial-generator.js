/**
 * @param {number} n
 * @yields {number}
 */
function* factorial(n) {
    let output = 1
    let count = Math.min(n, 1)

    while(count <= n) {
        yield output
        output *= ++count
    }
    
};

/**
 * const gen = factorial(2);
 * gen.next().value; // 1
 * gen.next().value; // 2
 */