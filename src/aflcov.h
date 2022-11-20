#pragma once
#include "litecov.h"
#include "coverage.h"
#include <vector>
#include "cxx.h"
class AFLCov : public LiteCov
{
public:
    void GetCoverage(Coverage &coverage, rust::Vec<uint64_t> &afl_coverage, bool clear_coverage);
};
