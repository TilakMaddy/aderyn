/*

Let's create 3 fake detectors ALL of which are trying to optimize gas by pointing out that the property on a storage variable should when acesses in a loop should be cached especially if the said property doesn't change after every step in the for loop

Now we want to find out which one offers the best description. We do so by delegating the task to an LLM
but not fully left to its own devices.

As the end users of these detectors, we can control how the LLLM should judge the answers based on what we are looking for. We do so by asking a bunch of questions to these `descriptions`.

How it works
------------

If there are `n` detectors to test against and `q` questions, there will be `q * 2 * n * (n - 1) / 2` a.k.a `q * 2 * (n choose 2)` battles.

For each question, for each bi-combination (length 2) set of detectors, we make LLM pick the "better" description. However since large language models have a bias on the position of the description presented to it, we make the detectors battle again where we flip the position of the descriptions and re-ask the same question.

The above trick is borrowed from the paper `Peer Rank and Discussion Improve Large Language Model based Evaluations` published at https://arxiv.org/abs/2307.02762 where the authors tried to evaluate various LLMs, here we do that to detectors' descriptions.

The default values for elo rating are picked up from https://colab.research.google.com/drive/1lAQ9cKVErXI1rEYq7hTKNaCQ5Q8TzrI5?usp=sharing#scrollTo=QLGc6DwxyvQc

More on how Elo Rating works can be found here https://en.wikipedia.org/wiki/Elo_rating_system

Caveat: Do not make the question too descriptive to answer for the following reason. When LLMs answer via chain of thought, the length could get too big that canclout judgement because of the phenomenon of "lost in the middle". More about that can be found here - https://arxiv.org/abs/2307.03172


*/

use std::collections::HashMap;

use aderyn_driver::detector::Detector;
use aderyn_evaluator::llm::detector_description;

#[derive(Default)]
pub struct DummyDetectorA;

#[derive(Default)]
pub struct DummyDetectorB;

#[derive(Default)]
pub struct DummyDetectorC;

impl Detector for DummyDetectorA {
    // Below text has been copy-pasted from code4rena automated findings.
    // https://github.com/code-423n4/2023-12-initcapital/blob/main/4naly3er-report.md
    fn description(&self) -> String {
        String::from(
            "If not cached, the solidity compiler will always read the length of the array during each iteration. That is, if it is a storage array, this is an extra sload operation (100 additional extra gas for each iteration except for the first) and if it is a memory array, this is an extra mload operation (3 additional gas for each iteration except for the first)."
        )
    }
}

impl Detector for DummyDetectorB {
    fn description(&self) -> String {
        String::from("Array length will be read from storage in every loop and it can be expensive")
    }
}

impl Detector for DummyDetectorC {
    fn description(&self) -> String {
        String::from("Array length not cached in loop.")
    }
}

fn main() {
    // The 3 detectors will be our contestants

    let mut contestants: HashMap<String, Box<dyn Detector>> = HashMap::new();

    contestants.insert("det-A".to_string(), Box::<DummyDetectorA>::default());
    contestants.insert("det-B".to_string(), Box::<DummyDetectorB>::default());
    contestants.insert("det-C".to_string(), Box::<DummyDetectorC>::default());

    // How we want the descriptions of our "ideal" detector of this type
    // List out what makes us satisifed about the description

    let satisfiers = vec![
        // These are the "questions" referenced above
        "It informs the user about specific details such extra gas cost that can be saved"
            .to_owned(),
        "It tells what has gone wrong".to_owned(),
    ];

    detector_description::run(contestants, satisfiers);
}
