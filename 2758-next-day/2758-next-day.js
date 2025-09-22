/** 
 * @return {string}
 */
Date.prototype.nextDay = function() {
    const next = this.getTime() + 86400000;
    return new Date(next).toISOString().split("T")[0];
}

/**
 * const date = new Date("2014-06-20");
 * date.nextDay(); // "2014-06-21"
 */