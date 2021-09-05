const fs = require("fs");
const path = require("path");

const soprox = path.join(__dirname, "../soprox.config.json");
if (fs.existsSync(project)) {
  const conf = require(soprox);
  if (conf?.mainnet?.payer?.secretKey || conf?.mainnet?.program?.secretKey)
    throw new Error(
      "CAUTION! Remove secret key in mainnet config to commit the repo."
    );
}
