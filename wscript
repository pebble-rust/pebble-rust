#
# This file is the default set of rules to compile a Pebble project.
#
# Feel free to customize this to your needs.
#

from waflib import TaskGen

top = '.'
out = 'build'

TaskGen.declare_chain(
    name='rustc',
    rule='${RUSTC} +nightly -L ../lib --target thumbv7m-none-eabi ${SRC} --emit=llvm-ir -A dead-code -o ${TGT}',
    ext_in=['.rs'],
    ext_out=['.ll'])

TaskGen.declare_chain(
    name='llc',
    rule=('${LLC} -mtriple=thumbv7m-none-eabi -relocation-model=pic -march=thumb '
          '-mattr=+thumb2 -mattr=+soft-float -mcpu=cortex-m3 --asm-verbose=false '
          '-o ${TGT} ${SRC}'),
    ext_in=['.ll'],
    ext_out=['.s'])

TaskGen.declare_chain(
    name='as',
    rule='${AS} ${ASFLAGS} -c ${SRC} -o ${TGT}',
    ext_in=['.s'],
    ext_out=['.o'])


def options(ctx):
    ctx.load('pebble_sdk')


def configure(ctx):
    ctx.load('pebble_sdk')


def build(ctx):
    ctx.load('pebble_sdk')

    binaries = []

    for p in ctx.env.TARGET_PLATFORMS:
        ctx.set_env(ctx.all_envs[p])
        ctx.env.RUSTC = 'rustc'
        ctx.env.LLC = 'llc'
        ctx.set_group(ctx.env.PLATFORM_NAME)
        app_elf = '{}/pebble-app.elf'.format(ctx.env.BUILD_DIR)
        ctx.pbl_program(source=ctx.path.ant_glob('src/*.(rs|c|o)'), target=app_elf)
        binaries.append({'platform': p, 'app_elf': app_elf})

    ctx.set_group('bundle')
    ctx.pbl_bundle(binaries=binaries, js=ctx.path.ant_glob('src/js/**/*.js'))
