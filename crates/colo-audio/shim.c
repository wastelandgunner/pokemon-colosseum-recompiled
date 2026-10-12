/* Thin C layer over the MusyX runtime for colo-audio: plain-argument entry points, and the one global
 * ReverbHI the game keeps (0x80452500 in main.dol). */
#include <musyx/musyx.h>
#include <musyx/pc.h>
#include <stdlib.h>
#include <string.h>

static SND_AUX_REVERBHI g_rev;
static void* hook_malloc(size_t n) { return aligned_alloc(32, (n + 31) & ~(size_t)31); }
static void hook_free(void* p) { free(p); }

/* sndSetReceiveMessageCallback (the game's 0x80167040 releases the work whose handle sent it): queued here
 * and drained by the Rust layer. */
static unsigned g_msgs[256];
static unsigned g_nmsg;
static void msg_cb(SND_VOICEID vid, s32 mesg) { (void)mesg; if (g_nmsg < 256) g_msgs[g_nmsg++] = vid; }
unsigned colo_snd_take_messages(unsigned* out, unsigned max) {
  unsigned n = g_nmsg < max ? g_nmsg : max;
  memcpy(out, g_msgs, n * sizeof *out);
  memmove(g_msgs, g_msgs + n, (g_nmsg - n) * sizeof *out);
  g_nmsg -= n;
  return n;
}

/* sndInit(64, 48, 16, 1, 0, 0x9FC000) like the game (0x80166E88), rendering at `rate` Hz, `channels` 1/2.
 * ReverbHI on aux A of studio 0 (sndSetAuxProcessingCallbacks(0, ReverbHI, &rev, 0xFF, 0, NULL, 0, 0xFF, 0)). */
int colo_snd_init(unsigned rate, unsigned channels) {
  static SND_HOOKS hooks = {hook_malloc, hook_free};
  SND_PC_CONFIG cfg = {rate, channels};
  if (!sndPCConfigure(&cfg)) return -1;
  sndSetHooks(&hooks);
  if (sndInit(64, 48, 16, 1, 0, 0x9FC000) != 0) return -2;
  memset(&g_rev, 0, sizeof g_rev);
  g_rev.coloration = 0.0f; g_rev.mix = 1.0f; g_rev.time = 1.8f; g_rev.damping = 0.0f; g_rev.preDelay = 0.02f; g_rev.crosstalk = 0.0f;
  sndAuxCallbackPrepareReverbHI(&g_rev);
  sndSetAuxProcessingCallbacks(0, sndAuxCallbackReverbHI, &g_rev, 0xFF, 0, NULL, NULL, 0xFF, 0);
  sndSetReceiveMessageCallback(msg_cb);
  return 0;
}

/* _sndSetReverbParm + sndAuxCallbackUpdateSettingsReverbHI. */
void colo_snd_set_reverb(float coloration, float mix, float time, float damping, float preDelay, float crosstalk) {
  g_rev.tempDisableFX = 0;
  g_rev.coloration = coloration; g_rev.mix = mix; g_rev.time = time; g_rev.damping = damping;
  g_rev.preDelay = preDelay; g_rev.crosstalk = crosstalk;
  sndAuxCallbackUpdateSettingsReverbHI(&g_rev);
}

int colo_snd_push_group(void* proj, unsigned gid, void* samples, void* sdir, void* pool) {
  return sndPushGroup(proj, (SND_GROUPID)gid, samples, sdir, pool) ? 1 : 0;
}

/* sndSeqPlayEx(group, song, data, {flags = SND_PLAYPARA_VOLUME, speed 0x100, volume {time, target}}, 0). */
unsigned colo_snd_seq_play(unsigned gid, unsigned sid, void* data, unsigned fade_ms, unsigned vol) {
  SND_PLAYPARA para;
  memset(&para, 0, sizeof para);
  para.flags = SND_PLAYPARA_VOLUME;
  para.speed = 0x100;
  para.volume.time = (u16)fade_ms;
  para.volume.target = (u8)vol;
  return sndSeqPlayEx((SND_GROUPID)gid, (SND_SONGID)sid, data, &para, 0);
}

void colo_snd_seq_volume(unsigned vol, unsigned ms, unsigned seq, unsigned mode) { sndSeqVolume((u8)vol, (u16)ms, seq, (u8)mode); }
void colo_snd_seq_stop(unsigned seq) { sndSeqStop(seq); }
void colo_snd_seq_pause(unsigned seq) { sndSeqPause(seq); }
void colo_snd_seq_continue(unsigned seq) { sndSeqContinue(seq); }
int colo_snd_seq_valid(unsigned seq) { return sndSeqGetValid(seq) ? 1 : 0; }
unsigned colo_snd_fx_start(unsigned fx, unsigned vol, unsigned pan, unsigned studio) { return sndFXStartEx((SND_FXID)fx, (u8)vol, (u8)pan, (u8)studio); }
int colo_snd_fx_ctrl(unsigned vid, unsigned ctrl, unsigned value) { return sndFXCtrl(vid, (u8)ctrl, (u8)value) ? 1 : 0; }
int colo_snd_fx_keyoff(unsigned vid) { return sndFXKeyOff(vid) ? 1 : 0; }
unsigned colo_snd_fx_check(unsigned vid) { return sndFXCheck(vid); }
void colo_snd_volume(unsigned vol, unsigned ms, unsigned group) { sndVolume((u8)vol, (u16)ms, (u8)group); }
void colo_snd_output_mode(unsigned mode) { sndOutputMode((SND_OUTPUTMODE)mode); }
/* Interleaved s16 frames. */
int colo_snd_render(short* out, size_t frames) { return sndPCRender(out, frames) ? 1 : 0; }

/* The debugger (musyx/src/musyx/debugger, not built) instruments the runtime through these. */
#include <musyx/debugger.h>
void musyxDebuggerRuntimeFrame(u32 deltaTime) { (void)deltaTime; }
void musyxDebuggerRuntimeMacroStep(SYNTH_VOICE* voice, const MSTEP* step) { (void)voice; (void)step; }
void musyxDebuggerRuntimeVoiceStart(SYNTH_VOICE* voice) { (void)voice; }
void musyxDebuggerRuntimeVoiceStop(SYNTH_VOICE* voice) { (void)voice; }
void musyxDebuggerRuntimeMessage(u32 voiceId, s32 message) { (void)voiceId; (void)message; }
void musyxDebuggerSetPaused(bool8 paused) { (void)paused; }
bool8 musyxDebuggerIsPaused(void) { return 0; }
bool8 musyxDebuggerConsumeStepFrame(void) { return 0; }

/* sndPCValidateGroup with sizes: writes "section+offset: reason" into msg on failure. */
#include <stdio.h>
int colo_snd_validate_group(const void* proj, size_t proj_n, const void* pool, size_t pool_n, const void* sdir, size_t sdir_n,
                            const void* samp, size_t samp_n, char* msg, size_t msg_n) {
  SND_PC_GROUP_ASSETS a = {{proj, proj_n}, {pool, pool_n}, {sdir, sdir_n}, {samp, samp_n}};
  SND_PC_ASSET_ERROR e = {0};
  if (sndPCValidateGroup(&a, &e)) return 1;
  snprintf(msg, msg_n, "%s+%#zx: %s", e.section ? e.section : "?", e.offset, e.reason ? e.reason : "?");
  return 0;
}
