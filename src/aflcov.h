#pragma once
#include "litecov.h"
#include "coverage.h"
typedef std::vector<ModuleCoverage> VecCoverage;
class AFLCov : public LiteCov
{
public:
    void GetCoverage(Coverage &coverage, bool clear_coverage);
};
