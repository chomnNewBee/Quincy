Object.defineProperty(Array.prototype, 'x', {
    get: function () {
        return this[0]
    },
    set: function (val) {
        this[0] = val
    }
})

Object.defineProperty(Array.prototype, 'y', {
    get: function () {
        return this[1]
    },
    set: function (val) {
        this[1] = val
    }
})

Object.defineProperty(Array.prototype, 'z', {
    get: function () {
        return this[2]
    },
    set: function (val) {
        this[2] = val
    }
})