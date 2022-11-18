#pragma once
#include "litecov.h"
#include "coverage.h"
#include <string>
#include <set>
#include <list>

typedef std::vector<ModuleCoverage> VecCoverage;
class AFLCov : public LiteCov
{
public:
    void GetCoverage(VecCoverage &coverage, bool clear_coverage);
    void IgnoreCoverage(VecCoverage &coverage);
};

ModuleCoverage *GetModuleCoverage(VecCoverage &coverage, std::string &name);

void PrintCoverage(VecCoverage &coverage);
void WriteCoverage(VecCoverage &coverage, const char *filename);

void MergeCoverage(VecCoverage &coverage, VecCoverage &toAdd);
void CoverageIntersection(VecCoverage &coverage1,
                          VecCoverage &coverage2,
                          VecCoverage &result);
// returns coverage2 not present in coverage1
void CoverageDifference(VecCoverage &coverage1,
                        VecCoverage &coverage2,
                        VecCoverage &result);
// returns coverage2 not present in coverage1 and vice versa
void CoverageSymmetricDifference(VecCoverage &coverage1,
                                 VecCoverage &coverage2,
                                 VecCoverage &result);
bool CoverageContains(VecCoverage &coverage1, VecCoverage &coverage2);

void ReadCoverageBinary(VecCoverage &coverage, char *filename);
void ReadCoverageBinary(VecCoverage &coverage, FILE *fp);
void WriteCoverageBinary(VecCoverage &coverage, char *filename);
void WriteCoverageBinary(VecCoverage &coverage, FILE *fp);
