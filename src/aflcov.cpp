#include "aflcov.h"
#include <iostream>
void AFLCov::GetCoverage(Coverage &coverage, rust::Vec<uint64_t> &afl_coverage, bool clear_coverage)
{
    CollectCoverage();
    for (ModuleInfo *module : instrumented_modules)
    {
        ModuleCovData *data = (ModuleCovData *)module->client_data;

        if (data->collected_coverage.empty())
            continue;

        // check if that module is already in the coverage list
        // (if the client calls with non-empty initial coverage)
        std::cout << module->module_name << std::endl;
        ModuleCoverage *module_coverage =
            GetModuleCoverage(coverage, module->module_name);
        if (module_coverage)
        {
            module_coverage->offsets.insert(data->collected_coverage.begin(),
                                            data->collected_coverage.end());

            // Copy the coverage to afl_coverage
            std::copy(data->collected_coverage.begin(), data->collected_coverage.end(), std::back_inserter(afl_coverage));
            // printf("afl_coverage.size() = %d")
        }
        else
        {
            coverage.push_back({module->module_name, data->collected_coverage});
            std::copy(data->collected_coverage.begin(), data->collected_coverage.end(), std::back_inserter(afl_coverage));
        }
    }

    if (clear_coverage)
        ClearCoverage();
}
