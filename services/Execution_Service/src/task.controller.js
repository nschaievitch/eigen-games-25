"use strict";
const { Router } = require("express")
const CustomError = require("./utils/validateError");
const CustomResponse = require("./utils/validateResponse");
const imageService = require("./processImage");
const dalService = require("./dal.service");
const fs = require("fs")
const router = Router()
 

router.post("/execute/:image_cid", async (req, res) => {
    console.log("Executing task");

    try {
        const image_cid = req.params.image_cid;
        var taskDefinitionId = Number(req.body.taskDefinitionId) || 0;
        console.log(`taskDefinitionId: ${taskDefinitionId}`);
        const image = await dalService.getIPfsTask(ipfsHost + image_cid);
        
        // write image to enc.b64 file
        fs.writeFileSync("enc.b64", image)



        imageService.processImage();

        // read proc.b64 file to `result`
        const result = fs.readFileSync("proc.b64", {encoding : "utf8"})

        // publish proc.b64 to ipfs

        const cid = await dalService.publishJSONToIpfs({image: result});
        const proof_of_task = JSON.stringify({
            processed: cid,
            original: image_cid
        })

        const data = "";
        await dalService.sendTask(proof_of_task, data, taskDefinitionId);
        return res.status(200).send(new CustomResponse({proofOfTask: proof_of_task, data: data, taskDefinitionId: taskDefinitionId}, "Task executed successfully"));
    } catch (error) {
        console.log(error)
        return res.status(500).send(new CustomError("Something went wrong", {}));
    }
})


module.exports = router
