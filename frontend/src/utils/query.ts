export function get_query (name) {
    if(!global.window) throw new Error('Cannot get URL query string on server')
    let { search } = window.location
    let queryObj = Object.fromEntries(search
        .substr(1, search.length - 1)
        .split('&')
        .map(val => val.split('=').map(val => decodeURI(val)))
    )
    return queryObj[name]
}