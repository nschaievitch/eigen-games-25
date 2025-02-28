require('dotenv').config();
const axios = require("axios");
const { execSync } = require("child_process")


async function processImage() {
  execSync("./operator-lib")
}
  
  module.exports = {
    processImage,
  }