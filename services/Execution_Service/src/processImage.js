require('dotenv').config();
const axios = require("axios");
const { exec } = require("child_process")


async function processImage() {
  exec("./operator-lib")
}
  
  module.exports = {
    processImage,
  }