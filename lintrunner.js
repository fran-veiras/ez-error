import { ESLint } from "eslint"
import fs from  'fs'

(async function main() {
  // 1. Create an instance.
  const eslint = new ESLint();

  // 2. Lint files.
  const results = await eslint.lintFiles(["**/*.ts", "**/*.tsx"]);

  // 3. Format the results.
  const formatter = await eslint.loadFormatter("stylish");
  const resultText = formatter.format(results);

  // 4. Output it.
  console.log('hola', resultText);
  
  fs.writeFile("./testing.txt", `${resultText}`, function(err) {
    if(err) {
        return console.log(err);
    }
    console.log("The file was saved!");
  });
})().catch((error) => {
  process.exitCode = 1;
  console.error('mostrame', error);
});

