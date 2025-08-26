/**
 * @param {number} times
 * @return {string}
 */
String.prototype.replicate = function(times) {
    if (times ==0 ){
        return "";

    }
   return this.repeat(times)
}