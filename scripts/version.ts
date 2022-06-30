import { execSync } from "child_process";
import { writeFileSync, readFileSync } from "fs";

const TC = "src-tauri/tauri.conf.json";

const r = new TextDecoder().decode(execSync("git describe --abbrev=0 --tags"));
const version = r.trim().replace("v", "");

if (version && version !== "") {
  const conf = JSON.parse(new TextDecoder().decode(readFileSync(TC)));
  conf.package.version = version;
  writeFileSync(TC, JSON.stringify(conf, null, 2));
}
