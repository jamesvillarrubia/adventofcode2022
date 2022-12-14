// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->
use std::collections::HashSet;
use std::str::Split;


// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called
//     let input: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg
// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw";
    let input: &str = "vJrrdQlGBQWPTBTF
fcpTMnMqMfTnZpgMfPbFBWzHPpBPzbCPPH
mcVMfcsqZgmgVcmfgcmZmqZNJhrlrdhNhDdrRRJSvDTRhJlD
pMFRmLwHMbRPmMbPPddvqqrrNSTFVttdrN
hgfpgCGZcjpcgfvNtdrtjvrdtSrd
gZgsBBBlZggBGhsfhpzlzLDLmLRDRMLDPw
hChhMFCvqtTMwbSSHgTZWHZd
jjBNPjJJNfsNjVnVJJNcNfPwGbSbDZnZZgHrddwHrrgWGb
mBBRRmBBQBmNJWhCzqllzhRCCv
lQgpngNgQvHdSgWwjMRmDjmMDHmm
zCLVzfzzbbCzsZZPbZPLfFJJMDWWRcDsmJRcjmwTmlRj
BblftzBtlFznptSQQQppNG
wJJwqCtCGRcVlqlM
BQpppjBQLczTrvHRjH
QQQFnmQWWRfnpQBpQpfDCwCdbPDCwbNNPtdJPCZw
gpJjshBpgjZGppJqBGJjJZzTwzTmcvzwwcmvwsCFdmcF
WPSSWWSQLVdDDfWltDWLPfttvFCVmzCCTFnmwcnnnCTCzVVv
tLldLltDQfflrRWNqBRjgHBpJNZjHj
bzVJjVnjbCGVLZQLmmsJZZQQ
RrwwzhPScWSwrhvZZvZlZvvSTsQS
HwFhzFWdPHfcPwPWPdhWffnngpjtnjgtpnfGCGnG
CPwQtftDQfPDBPBCfDDDCDptszcpVVddcRczVLVdccRGrLWs
FqlSnhlqhmhMbHqqSQlHbcrRrsWzRdzdWVzLRGRrVF
MbQmSnHZhqZMQMTJCttjCgPCwfgDwT
CCSpvHtZHSwftFtdtbfR
QJmNPmjjJNgNVNSzDlmRqbWlqWWfcqfWqbwfqR
MDhJzmSMDmsZrLhssrvh
ZhznnrLzcHhHSjsjSBSsBnSS
dTwqDdqDRQjNdwqjldggDvBJfmBfTSBbBSvfSJsmff
DjCldNglVwFVgZHHHzhCMcLhMC
vBnShjwwSshmQPmtJLpJtn
rDLFCWrClWCMWWVrbFVJqpQqpHmtbzJPQzJmzb
ZMCCDCMNrTWTrgScgGRhsvcsGLSG
LQpJglQQRjQsppfsQbjfbQlBgBhFhrvghhZCdPZZZthPgv
zVHDMSWVVMVWDzmnVMHDSMMzZvNFFrFvPCdmdFdNdrBPZhhd
ncqqSzMVCcGWVSCTWCDcVTffLLLbJsLLsRLblRQTps
zjGzLQtFzzRgwwLhVrqw
dfClCdHZsmffZDWlBZHCDBmhJbqTgbwgqbTnwrgrqRbT
HsdwPCsWDpDsBpfdWdHldWpsGvvccPvGcvzQvFQvccjNztQt
wmVVgFPrFdwJrlNHQHSHCCHL
tWBtvnBqZZMpcvmmqMBRCQQLCLWCHfNQQCSGWL
BBsnmcpqPswFsTws
BQRpFPJJJJPmPRqZNFVhcczHHzggwjBhghgzHw
snTsTLtrlvSSSslsGdcwmjhgggvjHhmH
nSsWWTtCbbDSnlRRZCfFRNZmCVJf
tRRMCWLtJWQLqLrmLHVLqmqh
JszPjSbGbsnGnSZprVpFhvFqvhrqZZ
zgbGSDBbPsTgbDBzzSDnDnPTcWlWJCQlNttNwwwMcMctcW
hlVRvPvzqqtRdwRRJsst
HHVNVBMBjHLTjVjwDjsbjJwbdmbbss
NBZHMCNVCCpgLTWggBpgNLTvqCPnGhQhCCGlqlhfcvvGfn
mwbfbfbDCqRJZRbCSvmfWTQFczTznnnFTFnrzJsz
BdhHlLjpjjjncSprnsSFWS
ljhVBPjSdHPRfZRZDvVRZM
tGqbqBSsntRgNqwNNVVHVN
hclFvJZvCDFppDpZpHNggMTwdlrMQNVgHM
CCFmcZLDFpvzZhCFJvZvmwjSRWsBLWsnfWjRRGGfnsst
GdGQQFdcMPwMdLFvWsNWFDLfss
SqjhtjnrbVznSztSqtzpjztVvTmNNmmfMMfDDMDDNTqfgsqv
rVhhZjppVrhpVzRbjSnzHPMwlJCJQdcRPPlPPcHJ
JVQLVgVZghFtFlhghtvSzsddmrdvssmzSWtd
HTMJCBMCjnwNBnCbTNwMdWfzvzsrsvffWbdsmfPr
BGnnpDnTjjHJwDBNpqlFQVQFQclcRFQqpR
BRhbrQDttbTTtRDtTRRzLHWZLZHGSqWLCBNWqLNL
fwFPPSjmsmJGLsNH
dvfSdvfVMjPTttTzczgTcd
dZgggwzgvsggtdstZTZgdfnhSJSSJDDhnDBdppnGnhSp
VQWRQWVCRLFGnThJCpFJ
LbmmbQVcHcmmlWjmVlVQNVRzvwwZvTrsrNwNwzZrMvfsqt
lDZQSlHDbLccRPQhCNhG
gtsntgvdvBvvqgsTgBggdrWRNzPhWczPbWcdWCccGGGP
sTBttvrFnnMTMJngbqfLlZLpwFVljwppZZDl
zNNNgqpgmLgqlHBHsMGslH
WdWFrFwhcwWRwhddcRWcdQbcDDslzBDszsHbGBDfbHfzVlVl
ZvhRrvPQwvWFQRZvFdJttSPgCmNppCNzJnJS
fCzRRNGfqNRvwpQhwrGcwZZT
gJnStgMmLhdHndSSTjcTrTpcmrjjcrrw
FFJBbdddFPPhFFNWCF
btrHRSBBSNLLRPLwhbhpqpfWhQppWZ
zCzTvvmgDvgDZhqWZZthtDZh
ttTjMsvCgRRLRSsBRG
LsSFFTTDWdCsmFTlLSsLDDRRQCvhpRQGNGQBJBhGGMNB
zqPtqZnjPPrPvJHBMHrJrMpv
VbqfjZfwgtfjPgZPgtwDLTLcTlcFdWLdcdVTJF
pfMCDmpHbdMQQdQFFG
gdjldRsVFRntQnqR
rlJVsWgWPWjsslSpDbScmSDPHfCd
lnFFGgBFBslDFGbFSjnNTjjppSrQHhnT
zcvmCRcvZmcZzWpTQhQrrTSPtHWH
CRccrZJmdJlwDJwgswGg
hllrrDzggGppgSSLNWgW
jlTlPwwqjjntVpWWPNnP
wjjJqvQjJjQJbTjlFqhBMzfDDmMCGBMHDCGb
jvQPhhtCRtfmqHHjqHHJsl
FFSTcBTBTMwFGCTwMTcGwTVnsHSJzqqJJJplmlpJHszZZzZD
dLMdVMNGBdGFMTNTRRLrQWCQhgWQbhgf
gdRgdgzzrvrzggDwgDGpPLzrbNljMTsbWWjWjZbTjLZMWcWj
tFfCQHJJnJMJTJjNNMjl
HmtffVttqHQmBCBQCqfFnCwRqpDvPRrGppRggNzdwgzp
DHSqzQbzWlRLDzMZNpVLgnpNLggw
cZcdTmPPthPdsvvdhPGTvJgwnpgjjTgNNwMVngNBjNfn
PPdJPvrtGtcFdFFchDRHDqHzZWSQQCrQWQ
BcgnLBLsFvRnGRRRlzfJbbPJzwHPwPFz
hCDjWMDVNfVllfzddw
qqMqpWCMjDTWNWTBLpgsgLvZwtGLLg
zczPgpGzhnbmbchhHwqwhSwfwHCFWw
VJdmVLlLdVJSJWHSTFwH
rlttQLVLdvvZpgcGbmDrzGMD
WSvtpqqtqccttVQpVvJNJSVNCmTlnCWwTTnWlBBBjwCBTlTP
ZgfPHfPfMHsDCwnlGBwTMGBM
rgdffZhPrrLsdLZpvcFSJJNvpJhcJv
qVdqJGvzgJzJgwzgWvdJzpblcRRWmLFFcLBmllFRRMRFRH
TGGSsSssNPTSLlRLcPHMmnPB
tTjTZtNGhrCjQNCjQQDTCSjZvfJbdgqrdpwqfVzwgzvdvVgb
VTmwcTVSMHwbMwbDVBTcMpJfpfnWqdJbZpJldfsjZn
hNtPhtzFzPQGCCGFFCGtnqQqWZWplsjWdlnlldJn
vRCRzvvFFFvhrRthPtLrtNGSwBVDScDSgHHjwwcBgSgTSL
dWCsWbWWchblsmbWVZqqsSpsGfBqBVBB
DtTtjPJrgjjtTTwgPwwjrTgnLqSBZQLqngQppqnfBVQfGp
PJPwwtDwHGGJtJRFHmhCFRCvdmHR
mMsMJSCjllsSSmBBclsMsJHDbcHqqbHpqHGbDZHbqHpb
RnQnGVnzGzFQgzWzpzvpqDHW
QVhRTfGLLFGTTFFwhnQVNfFwJsJsMjsBMrlsjrJlPSPlTrls
JNMJSVSGVCjnWZMZWWcH
gLTcqbqhqbbgzgnjpnjjWHnP
wqlbcrfTwrvcLBwwRtJwsNRstRsCCN
MlBssQBchZDLNJZgmvGg
fdzHMfHSzSprfgSvvJbmvDGNDW
PCHTRfjHnzHMzzfrCPCpMTlFhcFstqVwVCFllQcBtqss
TtFnnFJfDhtdfJJcFtfnsfcFjBjLDjHrDLrCjMjwCLLrZjrS
qQmWmQzvWpRQGvgpGGRGRzmWwZMwBLCHMZjbBBCLwrHSLrqr
MRllgRWWMlsJFnlFclJT
SRRrRDRBRTdbdMRZBZMprTCJCnWGvChJGzLSWWzsGhCs
wwqHPtFwjwTHLHvGTsGW
FlPtqTNVcTVtwtmjRbBZfQbfZbQmRRMR
WSWfQttffsHSfRRRStfnCsQQqlJpbhnrnmNzJbzqNbbrpmnb
FGFPddBcBwDPzpzbWlpzDbbh
ZPdPPLMFdGwFFGdwGdZwcZgTtSTVCsRRSgSRTQWTTtCTtH
vHsfGHTvSvHHHsGHctMgtHrJwbJJwrjgbrdzjWCrdrrw
hqZRLmmZpFhcLhFmrzJQbzzmQQJWJJbm
LNZFcpPlhBRhqDDllRtnMssGfBsnttGTnttT
VDVrLrZZcjrhhFrZppGlglGMPFwFWNQw
bzszSBHBWNGcscpN
TJqBqSfTBBqBHzJqddBqzcLnLjnhCRTvvRrnDrvrvZRn
GLzrNWbtMptHDmNDglgmlD
fZtcfCRvtBcQjdjgmmjj
RhBhhqfSPPpttrnPnVVW
BhVRJGwWqtHjZqTDLZ
gQnfpBdPNpQrPNSfBdndnpTTDFZttDLLzZzTzCLNLZZD
mQQPsgrldpgdBQgSbGVcmcRwGMWhwVwW
DrLCctBCLQtSSQcLbcQHWvvvlWHHnWlWBlNRRB
wqdmpgqsZhzGphwwpZGsppRvfnJsWfHWvfFfWFsfvlNN
mwmhVppTqpGqpNZpqTbSLLttDrDDtPQTtr
qwqmgnglDnlgtQzQJzJQhmWQ
pTpTpssdsVvNsdTSZGdSdjvCRcqcRcVWVczhWChtchzWcR
sGTvPqZvSGdZZGdsvNGdPHrFHFBDlDLwPgBFLLBB
BBBGsGGBrBBrqWVqRnWBBBWpzFwMhjMFSFPzzSwPFPpzzFvg
HtCdDdDctZDtbHCffcbddbNfvjvFSPFjMhMgLwPgjbFhjFFj
NJTDdltNgCNDZJJZCDJZDfJtrWWnQGBqlRVVlrBsnlrqqmnr
PwZhgbZSWSqqGznv
tTPVVmptcsrNrsTNpjRzqfHvvGfGWjfjqGzHWn
RVRtVDRmsRtrctmJDtgBBhBhbFgJPFFMFJgP
jPzzCCPzTtTfzrRtgSNVRHvFQVvbpQppVN
sSnDlBGBwJbFNplVlN
cLwSwdMhSwcBcsBZgWjCTCWfCLffrg
RSNPvTTNqFTSvNrSBvBGJGzmFMslgCMJCgmzlc
fDVfpptLWQfnVLffVHbQDQCclJzGGCtGmmGJmzMshzGh
VfQnWZfZDbdnVHWcfWnfHWVvPrTSNZqSwSqPjjvBwRqrNS
FLRpmRwcpjfzjSnD
tGvPNvBnPQggPQQvPgNHDjSSjDzzthjzfHrjlT
JGqvWNCCGQBWGBQvVLsCMMRLRnRMnwMc
fGJbzgBffCGpPGDVnG
mcTccshvbbdRNRsNjdLjnVlHVnHLqVpDpDqD
wdmsWvWssbZTcWvRhfzMQtrzMgrfrZJgfQ
NfSbvZHZNRSbQbbQgZrMjhLwMrjLjwHLCmmh
NTWdJBFcWJFcdsFJqcqPwqmjpMrLCMpLMwLP
dNJctnFBVfSGgvnfZz
GSnRJfGfRJgMDMGWnfzdmptpFJppLvwLwvLt
hbjZzrQbblqcLtpwlHvFplTH
qrzqbschrQCqqjPcCVcCGDfGMWDgWNGDDSfgnf
vmMpCdTndCvMdmnFcCRJWBJGcZJRJB
NDNwGzshPLrwVVNsjswhGzjFSfFFQQRSJWRBFcFRfsWFQB
NwNhNjVzhhzzrgzdqqvqtnqvlqdggG
MdPLVSSlMMVMmlLBBLFdvZNWqWztStttRRNqzqNGTq
DhJfhghhCgwChJgJwHHzbsHpnZRtTWqqfZRGTnWTZtNqNRWR
hwHpJbprwpQhDHDCbCCzsClBvrLMVFPvmPlMMVMdLrvj
DssDrqRsWsNfzfsWLRzjgTdBlgzFpMlgTFTglT
ttCZnSQmSQmgjGQGQgDlBp
bhDnCmbwVmCwwtZttPwbRWsRJcqWJfcfsfrqVrqq
ldBgTMTRvBDVnCCCTdSRTqNjbjSbPPPPqtfPqtPJFJ
cZHZrszLrrrZHrbNjNtbJCfqNJLt
GZzCzWZGGsGzmzZcmGssZzZVvnVdBDddRRDnVlWgRTDdBM
RjNrrjwGDDqqGJsHtzpMHHGz
QCbWgbShmBCCPClmmWFHzJzTbDdsMJsTtpTD
fffQfnSCWDBfhCDLRrNrwcrqVqwNqn
zmRrDRzqjmLLHzDjLsHLflJlVVJlWWTDTfdMtlWJ
pPQQnbvSpvNbgfgfVtMVJfgdtG
SnpnVFcPnNnPvpNSFNSbhHLhrjhCqRsRBRrHCLhzmC
CZZzlnCZNlGGcbVrbtVlMtct
MgFQDFgQRLLHhJgDFqQJQLgdtVTrttSrPSmcbmTtvSqvVSTV
hFQDDfMDfLgHwWfBzWwwsZGW
bHVDdHVHTPMvnSQnWSDQgDmm
GhrCJfbfrhfbRJcqGqlwZtnBRtBWSQgQWWnWQW
lfcCrqJhlfFqphpplNCrNVMPMPLbsLPLzFVHVLsVdz
VDhFCZhtFdPqwwcp
SvnvHNNnTvbwNNgnHwTHgwBTLcdqmmfmqLGmmTRLPfpdGP
BNWsHJgSnwgMMgMBBWMDVJjtjZrDJZzztJhjQr
HDsSHLRnpjbpbbRDbqLjLjjGGVffMVGMdvnfMcNvfBBGcB
TCzQQztwwNTMqMdBVv
hCQWmtCzZthPPZPrLjSbJqjSjLLFjLpr
ZrrZqJDcZSCFLLHBFcjjHF
TgvnDTlTtQwgBfwwwzLjGLdF
VbnVngMtvDTTVMQDQMDQlsbZJChCmCPhprrZqhqZSZPJ
glMGHBJTJJTplgwcCgcqcFhhbWncFm
sSswtPfRDmWcCqfchq
RZSdSzsVzNPSwSSQsdzSSQpGLjJTMpBGrJrLLrplZBpG
WQqqwLqQlnlWDwtbVbtCNfVbpV
dFTRjBPhcBgBrFhTPhrbVptJpNNbbtJCbJSL
hjcmcRmgPPcRcPDmHHzGLWmsDmzH
rWFmrRmmccSZJWvSLZTH
hDPhGbhSjtbpqJLvJHjLHTqj
pnplBlfBPPhlgfDbDhglPMMrwrRRSSncwccQzddzmC
LbccJCGzbcCJcfGczcnmNnvNmZNLSDZZWPWS
dwstRhTsrsFddPZqvNWP
BBggRrQstBwBRTHWTprRCHHGVljfCGCfcljHjbGV
FHVBSVDvnsFDwwSVwwvGVSMFWhWcWptMWchWMtPPcWtNNWcj
TgqJrJTRmRCNrbcLjprLnp
qQTlfdlZQgmfqqnFVznvQwvnsBsV
TGpDDMQGMZNtfvDJdtWd
jbrmstmllRmNvVhmmvJVhv
tbrRzFFLlRrjFlLlTQgLQwwLMwgTZBTB
QFgFWQQfSgLFGmtnnVmqCPWmPH
TTzjgTbRRqnRsCPCsP
NDMMgZjzcJvbjhMcjZbbbJJNpdpBfBvSBBQwBSQLQSpSplBG
zcRNsQSSMjRsNNZZFBLQHHFFBPWF
tvwCtgvqLJNnNBCH
fNNwqrqNMpTrDlcs
MMHMVPRJHJWvqzWctbtQQdQz
DFfNFffDnTllfTfFfmzsjqcdtQGQpbddQQbssn
mlFNCgFNNNLrmLFCThhhzJBvhSJPVhMgMh
PWjhljbHFhjbFMWhjbPfhbTGZvlGcGlCLvvwtGCNZGvc
SRqBqBrmQWQrgQrrqrJBLZNccLNZmTCtvTGtCvCt
rJDzDSSBrzdqQWDPHFMjMFdjHMVnbM
qqLwvvtrLFqqfqrjjjdBZfBCBBJdlT
ZGZpRZHbQDzDWRRRVdBzSSlBdzjjzdJJ
ZGpgNDQmWGDRmRpZMQbvPPtnnFnLsstFmnFrPL
TdhcfZhdZZdpdbPWttCWrrCN
MBMMqRLgpGpFFWbNsvLwvCPCCP
mpBMnBRMBGqJfZcfZZHZlhfm
CdmGdnMcMwHjhDtFFnrj
vPbVbPBPPpgpgWJpvTjqDZZqSHqVZShrDj
BppjjgvbJjbpNbzPfNcGCLlCRcmLLflllGcc
qDtgVttGFtlslStS
gCZbbHCjvJbZjCbJhHhHJrZcslJcLzLllcLNFssMSsTlSM
CWbWrZgWBQQBBpfdPm
hstPtCGtltlTClllPJLScVdPdJjLPJMV
NHRbDZDQSDFFjjdJ
RqbQpgBmqZvqZNQqgZmmbszpTtthtCswhjslwwpTWC
CVdwBJJdppbbwdBVrJbrJbGPlMFSLrjrPjmPFFmPRRDF
NNWHHhNZTcQWhnNFlmSSlRmLjnPPRF
qWsccHTZccsNsZcvTcNtStpBtbdVwpfBwbVCBq
lPQHNJhMPMPFlNMHBqZBwQwQwQZwcCqw
bWddDzbWbftdDSDbgttgnSDWccLwcvBczqcqGZzLccZTZwwc
spWrssWrnDtDpfSWDtsFqlFPjNMjRJVVNNPJNp
bCCfcWVLTHfSSdHwhH
sGQSZSzQJmmQsphwHHHsndnpHN
zPSqrmZPFCvFTbWMLV
tLtVBGLJqGqVGbzGSCsSsSqQsFvZCSQv
gRgdWlHTBHgjjHlWpWjWjrwdCfQRZFSssQQQZmQmMSvZfFMQ
lPHlpWgjTldprNWHNNdHjTctcLbVcnNJJGbVzBhnbhhJ
zVrSwzzJbVrbqFCVVVwVCztWDDtfTZsWDZTLZZmSWsDm
bpgHlgBbbGGGglBGRNvMpWfTDjmDfWDjfRZZWLtmZs
bGbvQHMpQccFJPPh
VGqCPmPjfGqCdMqVMhjhmPChDJDJzvrrbrBvrdrpnJDpJDQr
TSRsgHRSFHTlHvJvBDvvzlptbv
ZFLTsRRgZTgWscHTfWNWNPPBfGqmmMfV
TvTrrrCVCVwqjPrWfWhjfH
RRmgmnggltRgNpzRsdfqWWjdFdvNHfdh
zZlRzDGGZmbmmZbvGJVccwCMVcVVTLwDwC
QPsNlvvvSccbbNQcSPvDVSvzTLLCgRVzCJgTJpgCpphgzh
MDqHwFrMffgFpgpJLzTz
ZtdrffBrdqmBBmfwMtDtQPPPbjcNvnllnlbNtScn
HbbbcpTHHMMqNTCddCVBQvgPzJPJWQBQjvpBvQ
FFrDGtntFFwhrRFDFthfRhRmSJPJvvJZjZjWJJvJQJnvWjjg
rtfFfLmLRmNgdHqcLNHd
FpFHCFWtFSnCWnBfJJfgMJDGHDGGsG
rhrLrrhLrbtZThLfgsfGNDfgTgNcDs
QmPjbdqjmbbbrmhQqQZrZStRdlnnFdlRzVVVWlnpzR
bBMwwjzhbjhssvsGZBSZLr
JFtnDtRzJtffJHWNtHncRRrvGZvSnllZZZsgvlnvVvlv
RRPHPHFPHHdcHtzNfMQhdCwbqCmbMChhqq
pWGdFSWwwjLdvgNNvggl
mTNbmRPHmmVNmvZhnhBssBlhnb
HPTzRPffJJNzjCFpDWDz
MHlgzsqHlbmzgsHzlsbcRWPdPtjZFqhGGdrPrjPJGrVP
vpwwvQwCnhNQpSnLdVtrrZGZtZjdVSdJ
hfffwvTpvLwDpCLvDnQDHbmRRTcWRHMWWHWMmWMW
WHlNHWWldjpwntnWBPpPQFZFBFhZBZCZ
TqqvgvmgfmvDVLLfqqLsrFBRhrrBFJQBGPgPZGCR
mcDbcDmzLcmDDzfVzTQNjNzNztdzjNdwSHlH";

    let split: Split<&str> = input.split("\n");
    let mut count = 0;

    // for s in split {
    //     let length = s.len();
    //     let half = length/2;
    //     let firstH = &s[0 .. half]; // make a slice of the string
    //     let secondH = &s[half .. length]; 

    //     let a: HashSet<char>  = firstH.chars().collect(); //convert slice to hashset
    //     let b: HashSet<char>  = secondH.chars().collect();

    //     // println!("{:?}", firstH);
    //     // println!("{:?}", secondH);
    //     let mut intersection = a.intersection(&b).take(1).last(); //use interection function of hashset
    //     // println!("{:?}",intersection);
    //     let inter = intersection.unwrap();
    //     println!("{:?}",inter);

    //     let dict = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    //     let g_index = dict.find(*inter).unwrap_or(0) + 1;   // 3
    //     println!("{:?}",g_index);
        
    //     count += g_index

    // }

    // println!("{}",count)
    
    let chunky: Vec<&str> = split.collect();
    let chunked = chunky.chunks(3);
    // println!("{:?}", chunked);


    for (i, el) in chunked.enumerate(){
        
        let a: HashSet<char>  = el[0].chars().collect(); //convert slice to hashset
        let b: HashSet<char>  = el[1].chars().collect(); //convert slice to hashset
        let c: HashSet<char>  = el[2].chars().collect(); //convert slice to hashset
        let mut inter1: String  = a.intersection(&b).collect();

        let d: HashSet<char>  = inter1.chars().collect();
        
        let mut inter2: String  = d.intersection(&c).collect();

        println!("The current index is {:?}", inter2);
        let dict = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let g_index = dict.find(&*inter2).unwrap_or(0) + 1;   // 3
        println!("{:?}",g_index);

        count += g_index

    }

    // Print text to the console
    println!("{}",count);


}