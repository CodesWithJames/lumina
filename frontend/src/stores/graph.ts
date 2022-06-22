export type GraphStore = {
    req: (type: TemplateStringsArray, ...text: any) => Promise<{ data: any, errors: string[]}>,
    auth_token?: string
}

export const graph_init = (auth_token?: string): GraphStore => {
    return {
        auth_token,
        async req (strings: TemplateStringsArray, ...variables) {
            let query = handleVariables(strings, variables)

            const BACKENDAPI = "api.albert.lumina.earth"
            const auth_token = this.auth_token;

            if(!BACKENDAPI) throw new Error('No domain provided')

            let request = new Request(`https://${BACKENDAPI}/graph`, {
                method: 'POST',
                body: query,
                headers: {
                    ...(auth_token ? { auth_token } : {})
                }
            })

            let response = await fetch(request)

            return await response.json()
        }
    }
}

function isPrimitive (test) {
    return (test !== Object(test))
}

function isArray (test) {
    return Array.isArray(test)
}

function isObject (test) {
    return typeof test === 'object' && !Array.isArray(test)
}

function parseArray (array) {
    return '[' + array.map(variable => parseVariable(variable)).join(',\n') + ']'
}

function parseObject (obj) {
    return '{\n' + Object.keys(obj).map(key => key + ': ' + parseVariable(obj[key])).join(',\n') + '\n}'
}

function parseVariable (variable) {
    if (isPrimitive(variable)) {
        return JSON.stringify(variable)
    }
    if (isArray(variable)) {
        return parseArray(variable)
    }
    if (isObject(variable)) {
        return parseObject(variable)
    }
    throw new Error('Could not translate: ' + variable)
}

function handleVariables (strings, variables) {
    let query = ''

    for (let index in strings) {
        query += strings[index]
        let obj = variables[index]
        if (obj) query += Object.keys(obj).filter(key => obj[key] !== undefined).map(key => key + ': ' + parseVariable(obj[key])).join(',\n')
    }

    return query
}