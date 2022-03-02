const b = require('benny')
const glob = require("util").promisify(require('glob'))
const globRs = require('..').glob
const fs = require('fs')

const PATTERN = "**/*.js"

module.exports = () => b.suite(
  "Glob",

  b.add('NAPI glob', async () => await globRs(PATTERN) ),

  b.add('JS glob', async () =>  await glob(PATTERN).then(arrs => 
    arrs.map(
      file => ({
        source: file,
        content: fs.readFileSync(file)
      })
    )
  )),

  b.cycle(),

  b.complete(),

  b.save({ file: "glob_result", version: "0.0.0"}),
  b.save({ file: "glob_result", format: "chart.html"}) 
)
