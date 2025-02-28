"use strict";
const { Router } = require("express")
const CustomError = require("./utils/validateError");
const CustomResponse = require("./utils/validateResponse");
const imageService = require("./processImage");
const dalService = require("./dal.service");

const router = Router()

router.post("/execute/:stringParam", async (req, res) => {
    console.log("Executing task");

    try {
        const stringParam = req.params.stringParam;
        var taskDefinitionId = Number(req.body.taskDefinitionId) || 0;
        console.log(`taskDefinitionId: ${taskDefinitionId}`);

        const result = await imageService.getPrice(stringParam);
        const cid = await dalService.publishJSONToIpfs(result);
        const data = "hello";
        await dalService.sendTask(cid, data, taskDefinitionId);
        return res.status(200).send(new CustomResponse({proofOfTask: cid, data: data, taskDefinitionId: taskDefinitionId}, "Task executed successfully"));
    } catch (error) {
        console.log(error)
        return res.status(500).send(new CustomError("Something went wrong", {}));
    }
})


module.exports = router
