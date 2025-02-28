require('dotenv').config();
const dalService = require("./dal.service");
const imageService = require("./processImage");
const fs = require("fs")
async function validate(proofOfTask) {

  try {
      const proofData = JSON.parse(proofOfTask);
      const processed = proofData.processed;
      const original = proofData.original;

      const taskResult = await dalService.getIPfsTask(processed);
      const originalImage = await dalService.getIPfsTask(original);

      // write image to enc.b64 file
      fs.writeFileSync("enc.b64", originalImage.image)

      imageService.processImage();

      // read proc.b64 file to `result`
      const data = fs.readFileSync("proc.b64", {encoding : "utf8"})
      
      //console.log("log", data)
      return data === taskResult.image;

    } catch (err) {
      console.error(err?.message);
      return false;
    }
  }
  
  module.exports = {
    validate,
  }