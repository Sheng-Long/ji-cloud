const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs-extra');

const {APP_NAME} = process.env;

const ALLOWED_APP_NAMES = [
	"user",
	"admin",
	"jig/edit",
	"jig/play",
	"module/memory/edit",
	"module/memory/play",
]

if(!APP_NAME) {
    console.error("requires APP_NAME in env");
    process.exit(1);
}

if(ALLOWED_APP_NAMES.indexOf(APP_NAME) === -1) {
	console.error(`${APP_NAME} is an invalid APP_NAME. Must be one of:`);
	console.error(`${JSON.stringify(ALLOWED_APP_NAMES)}`);
    process.exit(1);
}
//HTML
const srcPath = path.resolve("./dev-index.html");

const destDir = `../apps/dist/${APP_NAME}`;
const destPath = path.resolve(`${destDir}/index.html`);

fs.ensureDirSync(path.resolve(destDir));
fs.readFile(srcPath, 'utf-8')
    .then(html => html.replace("{{APP_NAME}}", APP_NAME))
    .then(html => fs.writeFile(destPath, html))
    .catch(err => console.error(err));

