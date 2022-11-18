#include "aflcov.h"
void AFLCov::GetCoverage(Coverage &coverage, bool clear_coverage)
{
    CollectCoverage();
    for (ModuleInfo *module : instrumented_modules)
    {
        ModuleCovData *data = (ModuleCovData *)module->client_data;

        if (data->collected_coverage.empty())
            continue;

        // check if that module is already in the coverage list
        // (if the client calls with non-empty initial coverage)
        ModuleCoverage *module_coverage =
            GetModuleCoverage(coverage, module->module_name);
        if (module_coverage)
        {
            module_coverage->offsets.insert(data->collected_coverage.begin(),
                                            data->collected_coverage.end());
        }
        else
        {
            coverage.push_back({module->module_name, data->collected_coverage});
        }
    }
    if (clear_coverage)
        ClearCoverage();
}
