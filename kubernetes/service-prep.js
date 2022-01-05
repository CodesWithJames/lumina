let { writeFileSync, readFileSync } = require('fs');
let yml = readFileSync('./kubernetes/remotedev/services.yml', 'utf8')

// turn node args into object with key/value pairs (eg: NAMESPACE=STAGING)
let args = process.argv.slice(2).reduce((acc, curr) => {
    let [key, value] = curr.split('=')
    acc[key] = value
    return acc
}, {})

// create a new yaml file with the environment variables replaced
writeFileSync('./kubernetes/generated/services.yml', yml.replace(/\$\{([^}]*)\}/g, (match, p1) => {
    return args[p1] || match
}), 'utf8')

process.stdout.write(`name: framework
kubectl:
  manifests:
  - kubernetes/generated/**`)