#! /bin/bash
echo "test AOF.."
source test_AOF.sh
echo ""
echo "test master slave.."
source test_master_slave.sh
echo ""
echo "test cluster.."
source test_cluster.sh
echo ""
echo "test multi.."
source test_multi.sh
echo ""
echo "test watch.."
source test_watch.sh