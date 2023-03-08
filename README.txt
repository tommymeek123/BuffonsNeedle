Western Carolina University
CS 370 Operating Systems
Spring 2021
Project 3 - Buffon's Needle
Authors: Tommy Meek and Hannah Young

usage: cargo run --release
Hint: Try using several million or billion needles to get a good estimate of pi.

The aim of this project is to simulate the famous "Buffon's Needle" problem.
The problem is stated as follows: "Suppose we have a floor made of parallel
strips of wood, each the same width, and we drop a needle onto the floor.
What is the probability that the needle will lie across a line between two
strips?"

The answer the the problem, assuming that the needles are no longer than the
width of the wooden strips is (2*l) / (pi*w) where l is the length of the
needles and w is the width of the wooden strips. This simulation will be used
to approximate the value of pi.
