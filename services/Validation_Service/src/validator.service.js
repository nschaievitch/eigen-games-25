require('dotenv').config();
const dalService = require("./dal.service");
const imageService = require("./processImage");

async function validate(proofOfTask) {

  try {
      const proofData = JSON.parse(proofOfTask);
      const processed = proofData.processed;
      const original = proofData.original;

      const taskResult = await dalService.getIPfsTask(processed);
      const originalImage = await dalService.getIPfsTask(original);

      var data = await imageService.processImage(originalImage.image);

      return data === taskResult.image;
    } catch (err) {
      console.error(err?.message);
      return false;
    }
  }
  
  module.exports = {
    validate,
  }