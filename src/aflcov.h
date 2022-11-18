#pragma once
#include "litecov.h"
#include "coverage.h"
#include <vector>
#include "cxx.h"
class AFLCov : public LiteCov
{
public:
    void GetCoverage(rust::Vec<uint64_t> &coverage, bool clear_coverage);
};
